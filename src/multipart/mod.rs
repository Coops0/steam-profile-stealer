mod multipart_form;
mod data_profile_edit;
mod profile_details;
pub mod update;

#[cfg(test)]
mod tests {
    use anyhow::{bail, Result};
    use crate::multipart::multipart_form::update_name;
    use crate::multipart::profile_details::new_name_details;

    const AUTH_COOKIE: &str = "76561198286609782%7C%7CeyAidHlwIjogIkpXVCIsICJhbGciOiAiRWREU0EiIH0.eyAiaXNzIjogInI6MEQzNV8yMjk2RTJBRV8xMDFERSIsICJzdWIiOiAiNzY1NjExOTgyODY2MDk3ODIiLCAiYXVkIjogWyAid2ViIiBdLCAiZXhwIjogMTY4NTM5ODk4MiwgIm5iZiI6IDE2NzY2NzE0MjUsICJpYXQiOiAxNjg1MzExNDI1LCAianRpIjogIjBEMzNfMjI5NkUyQUFfNEFDOEIiLCAib2F0IjogMTY4NTMxMTQyNCwgInJ0X2V4cCI6IDE3MDMzNjg5ODgsICJwZXIiOiAwLCAiaXBfc3ViamVjdCI6ICI3MS4xOTEuODQuMjgiLCAiaXBfY29uZmlybWVyIjogIjcxLjE5MS44NC4yOCIgfQ.krbOuPTuGDTSP7y0oDfr-YwLLQKGnFV_qke5m-dtgIfeWynjFH_MTvdJ4Be9j9uSmHwcZMxj7G3NA65G4fYFCQ";

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