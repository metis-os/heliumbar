// use futures::stream::StreamExt;
use upower_dbus::UPowerProxy;

pub fn power_info () -> zbus::Result<()>  {
futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        let upower = UPowerProxy::new(&connection).await?;

        let device = upower.get_display_device().await?;

        println!("Battery: {:?}", device.percentage().await);

        Ok(())
    })

}
