use crate::message::WebsocketWrapper;
use anyhow::Result;
use tokio::join;
use crate::multipart::multipart_form::{clear_aliases, update_image, update_name};
use crate::multipart::profile_details::new_name_details;

async fn update(wrapper: &mut WebsocketWrapper, name: &str, image_url: Option<&str>) -> Result<()> {
    let (session_id, fields) = new_name_details(wrapper, name).await?;

    let name = if let Some(image_url) = image_url {
        let (name, img) = join!(
            update_name(wrapper, &session_id, Vec::from(fields.clone())),
            update_image(wrapper, &session_id, image_url)
        );

        name
    } else {
        update_name(wrapper, &session_id, Vec::from(fields)).await
    };

    todo!("error handling");
    clear_aliases(wrapper, &session_id).await?;
    Ok(())
}