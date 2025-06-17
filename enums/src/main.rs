
//Enums
fn main() {


    #[derive(Debug)]
    enum IpAddressType{
        V4,
        V6
    }
    let _v4=IpAddressType::V4;
    let _v6=IpAddressType::V6;

    println!("IP Address v4 is:{:?}",_v4);

    struct IpAddress{
        ip_kind:IpAddressType,
        address:String
    }
    let _home:IpAddress=IpAddress{
        ip_kind:IpAddressType::V4,
        address:String::from("127.0.0.1")
    };

    let _loopback:IpAddress=IpAddress{
        ip_kind:IpAddressType::V6,
        address:String::from("::1")
    };

    #[derive(Debug)]
    enum IpAddr{
        V4(u8,u8,u8,u8),
        V6(String)
    }
    //Enhanced enums
    let _h=IpAddr::V4(127,0,0,1);
    let _lbk=IpAddr::V6(String::from("::1"));

    println!("IP V4 address is: {:?}",_h);
    println!("IP V6 address is: {:?}",_lbk);
}
