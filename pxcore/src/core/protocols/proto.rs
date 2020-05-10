use std::net::SocketAddr;



#[async_trait::async_trait]
pub trait Scannable: Send + Sync + std::fmt::Display {
    async fn scan(&self) -> Result<bool, Box<dyn std::error::Error>>;
}



// #[derive(Debug)]
// struct SocksScanner {
//     addr: SocketAddr
// }

// impl std::fmt::Display for SocksScanner {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Socks5")
//     }
// }

// #[async_trait::async_trait]
// impl Scannable for SocksScanner {
//     async fn scan(&self) -> bool {true}
// }