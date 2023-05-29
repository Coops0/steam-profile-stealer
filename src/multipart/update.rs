use crate::message::{SteamMessageOut, WebsocketWrapper};
use anyhow::{anyhow, Context, Result};
use tokio::join;
use crate::multipart::multipart_form::{clear_aliases, update_image, update_name};
use crate::multipart::profile_details::new_name_details;

async fn update(wrapper: &mut WebsocketWrapper, new_name: &str, image_url: Option<&str>) -> Result<()> {
    wrapper.log("Gathering profile data to use in name request...");
    let (session_id, fields) = new_name_details(wrapper, new_name)
        .await
        .context("Failed to get new name details")?;

    wrapper.log("Successfully got profile details!").await;

    let name = if let Some(image_url) = image_url {
        wrapper.log("Simultaneously updating image and name...").await;

        let (name, img) = join!(
            update_name(wrapper, &session_id, Vec::from(fields.clone())),
            update_image(wrapper, &session_id, image_url)
        );

        wrapper.log("Both functions finished, processioning responses...").await;

        let img = img.context("Failure updating image!")?;
        if img.success {
            wrapper
                .sm(SteamMessageOut::PictureChange { url: image_url.to_owned() })
                .await;

            wrapper.log("Successfully updated image!").await;
        } else {
            return Err(anyhow!(img).context("Failed to update image."));
        }

        name
    } else {
        wrapper.log("Only updating name...").await;
        update_name(wrapper, &session_id, Vec::from(fields)).await
    };

    let name = name.context("Failure updating name!")?;

    if name.success == 1 {
        wrapper
            .sm(SteamMessageOut::NameChange { name: new_name.to_owned() })
            .await;

        wrapper.log("Successfully updated name! Clearing aliases...").await;
        clear_aliases(wrapper, &session_id).await
            .context("Error clearing aliases!")?;

        wrapper.log("Successfully cleared aliases!").await;
    } else {
        return Err(anyhow!(o).context("Failed to update name!"));
    };

    Ok(())
}