use bluer::{
    DiscoveryFilter, AdapterEvent, Session, Uuid
};
use anyhow::{Context, Result};
use futures::StreamExt;
use tokio::time::timeout;

pub(super) async fn scan_devices(scan_seconds: u64) -> Result<()> {
    // Получаем Bluetooth-адаптер
    let session = Session::new().await.context("Нет сеанса, bluetoothd работает?")?;
    let adapter = session.default_adapter().await.context("Отсутвует адаптер")?;
    adapter.set_powered(true).await?;
    let serial_services = [Uuid::from_u128(0x00001101_0000_1000_8000_00805f9b34fb)];
    adapter.set_discovery_filter(DiscoveryFilter { uuids: serial_services.into(), ..Default::default() }).await.context("фильтр накрылся")?;
    // Сканирование устройств
    let mut discovery = adapter.discover_devices_with_changes().await?;
    println!("Scanning...");

    let start = std::time::Instant::now();
    let mut scan_time = std::time::Duration::from_secs(scan_seconds);
    let end = start + scan_time;
 
    while scan_time.as_secs() > 0 {
        match timeout(scan_time, discovery.next()).await {
            Ok(Some(AdapterEvent::DeviceAdded(addr))) => {
                if let Ok(device) = adapter.device(addr) {
                    println!(
                        "Устройство: {} [Имя: {:?}] Battery: {:?}%",
                        addr,
                        device.name().await?,
                        device.battery_percentage().await?,
                    );
                }
                scan_time = end - std::time::Instant::now();
            }
            Ok(_) => {
                scan_time = end - std::time::Instant::now();
            }
            Err(_) => {
                println!("Scanning finnished");
                break;
            }
        }
    }

    Ok(())
} 

