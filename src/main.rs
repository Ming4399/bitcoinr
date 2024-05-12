//! 比特币网络的地址定义
//!
//! 这个模块定义比特币地址
//!

use std::net::{Ipv4Addr,SocketAddr};


/// 用于表明地址提供的服务类型，元组结构体
#[derive(Debug)]
pub struct ServiceFlags(u64);


impl ServiceFlags {
    /// NONE 就是没有提供任何服务
    pub const NONE: ServiceFlags = ServiceFlags(0);

    /// NETWORK 代表该节点可以提供完整的区块链服务。未修剪区块链都是这么设置的，SPV客户端则要关掉
    pub const NETWORK: ServiceFlags = ServiceFlags(1 << 0);

    /// 用于提供 getUTXO 服务，定义于 BIP64, BITCOIN CORE 没有这个功能.
    pub const GETUTXO: ServiceFlags = ServiceFlags(1 << 1);

    /// BLOOM 意味着节点有能力并且愿意处理经过布隆过滤的连接。 
    /// 比特币核心节点过去默认支持此功能，无需广告此位，
    /// 但从协议版本 70011 (= NO_BLOOM_VERSION) 开始不再这样做
    pub const BLOOM: ServiceFlags = ServiceFlags(1 << 2);

    /// 表明节点是否支持隔离见证
    pub const WITNESS: ServiceFlags = ServiceFlags(1 << 3);

    /// COMPACT_FILTERS 表示节点将为基本块过滤器请求提供服务。 
    /// 有关如何实现的详细信息，请参阅 BIP157 和 BIP158。
    pub const COMPACT_FILTERS: ServiceFlags = ServiceFlags(1 << 6);

    /// NETWORK_LIMITED 与 NODE_NETWORK 含义相同，
    /// 但限制为仅服务最后 288 个（2 天）块。 
    /// 有关如何实现的详细信息，请参阅 BIP159。
    pub const NETWORK_LIMITED: ServiceFlags = ServiceFlags(1 << 10);

    /// P2P_V2表示节点支持P2P v2加密传输协议。 
    /// 有关如何实现的详细信息，请参阅 BIP324。
    pub const P2P_V2: ServiceFlags = ServiceFlags(1 << 11);
}

#[derive(Debug)]
pub struct Address {
    /// 表明地址对应的节点所提供的服务
    pub services: ServiceFlags,
    /// 8个16位数组，表示IPV4或IPV6地址
    pub address: [u16;8],
    /// 网络端口
    pub port: u16
}

impl Address {
    /// 新建一个地址给对应的socket
    pub fn new(socket: &SocketAddr, services: ServiceFlags) -> Address {
        let (address,port) = match *socket {
            SocketAddr::V4(addr) => (addr.ip().to_ipv6_mapped().segments(), addr.port()),
            SocketAddr::V6(addr) => (addr.ip().segments(),addr.port())
        };
        Address {address, port,services}
    }
}


fn main() {
    println!("Hello, Bitcoin!");
}

#[cfg(test)]
mod network_test {
    use crate::{Address, ServiceFlags};


    /// 测试序列化比特币地址
    #[test]
    fn serialize_address_test() {
        println!("{:?}",Address {
            services: ServiceFlags::NETWORK,
            address: [0, 0, 0, 0, 0, 0xffff, 0x0a00, 0x0001],
            port: 8333
        });
        assert!(1==2);
    }

}