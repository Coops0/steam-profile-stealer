use crate::message::{SteamMessageOut, WebsocketWrapper};
use anyhow::{bail, Context, Result};
use tokio::join;
use crate::multipart::multipart_form::{clear_aliases, update_image, update_name};
use crate::multipart::profile_details::new_name_details;

pub async fn update(wrapper: &mut WebsocketWrapper, new_name: &str, image_url: Option<&str>) -> Result<()> {
    wrapper.log("Gathering profile data to use in name request...").await;
    let (session_id, fields) = new_name_details(wrapper, new_name)
        .await
        .context("Failed to get new name details")?;

    wrapper.log("Successfully got profile details!").await;

    let fc = wrapper.fake_clone();

    let name = if let Some(image_url) = image_url {
        wrapper.log("Simultaneously updating image and name...").await;

        let dumb = fc.fake_clone();
        let ass = fc.fake_clone();

        let (name, img) = join!(
            update_name(dumb, &session_id, Vec::from(fields.clone())),
            update_image(ass, &session_id, image_url)
        );

        wrapper.log("Both functions finished, processioning responses...").await;

        let img = img.context("Failure updating image!")?;
        if img.success {
            wrapper
                .sm(SteamMessageOut::PictureChange { url: image_url.to_owned() })
                .await;

            wrapper.log("Successfully updated image!").await;
        } else {
            bail!("Failed to update img -> {img:?}");
        }

        name
    } else {
        let dude = fc.fake_clone();
        wrapper.log("Only updating name...").await;
        update_name(dude, &session_id, Vec::from(fields)).await
    };

    let name = name.context("Failure updating name!")?;

    if name.success == 1 || (name.success == 2 && name.err_msg == "An error occurred while setting account details<br />") {
        wrapper
            .sm(SteamMessageOut::NameChange { name: new_name.to_owned() })
            .await;

        wrapper.log("Successfully updated name! Clearing aliases...").await;
        clear_aliases(fc, &session_id).await
            .context("Error clearing aliases!")?;

        wrapper.log("Successfully cleared aliases!").await;
    } else {
        bail!("Failed to update name -> {name:?}");
    };

    Ok(())
}