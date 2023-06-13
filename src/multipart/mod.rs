mod multipart_form;
mod data_profile_edit;
mod profile_details;
pub mod update;

#[cfg(test)]
mod tests {
    use anyhow::{Result};
    use crate::message::WebsocketWrapper;

    use crate::multipart::profile_details::new_name_details;

    const AUTH_COOKIE: &str = "";

    #[tokio::test]
    async fn get_details_test() -> Result<()> {
        let mut wrapper = crate::message::WebsocketWrapper::new(None);
        wrapper.cookie = AUTH_COOKIE.to_owned();
        wrapper.profile.url = "https://steamcommunity.com/id/coops_".to_owned();

        let details = new_name_details(&mut wrapper, "joe").await?;

        println!("{details:?}");
        Ok(())
    }

    // #[tokio::test]
    // async fn update_name_test() -> Result<()> {
    //     let mut wrapper = crate::message::WebsocketWrapper::new(None);
    //     wrapper.cookie = AUTH_COOKIE.to_owned();
    //     wrapper.profile.url = "https://steamcommunity.com/id/coops_".to_owned();
    //
    //     let response = update_name(&mut wrapper").await?;
    //     println!("{response:?}");
    //
    //     if response.success == 2 && response.err_msg == "An error occurred while setting account details<br \\/>" {
    //         // It might have gone through? It updates but says it fails sometimes idk its weird
    //         return Ok(());
    //     }
    //
    //     if response.success != 1 {
    //         bail!("got non success response!");
    //     }
    //
    //     Ok(())
    // }
}