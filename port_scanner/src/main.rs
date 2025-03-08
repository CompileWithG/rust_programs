use std::env::args;
use std::net::TcpStream;
use std::time::Duration;

#[derive(Debug)]
struct Addr{
    flag:String,
    IPAddr:String,
}
fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    TcpStream::connect_timeout(&address.parse().unwrap(), Duration::from_secs(1)).is_ok()
}
fn main() {
    let arguments=args().collect::<Vec<String>>();
    let  flag: String;
    let ip_addr;
    if arguments.len()>1{
        flag = arguments[1].clone();
    }
    else{
        flag=String::from("-z");//here -z stands for no flag used
    }
    if arguments.len()>2{
        ip_addr= arguments[2].clone().split(".").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    }
    else{
        ip_addr=vec![256,256,256,256];//as 256  isnt valid we give this value to all sections of
                                      //the ip adresss to denote that the user hasnt given an ip
                                    // address input
    }
    println!("this is the ip address given by the user :{:?}",ip_addr);
    println!("this is the flag {}",flag);
    if ip_addr.len()!=4 {
        println!("invalid ip adress ,there should be only 4 octets in an ipv4_address ");
        panic!();
    }
    for i in ip_addr.iter(){
        if *i>255 || *i<0{
            println!("each octet value should be between 0 and 256 in an ipv4_address");
            panic!();
        }
    }
    let a=ip_addr.into_iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");
    for i in 1..1025{
        if scan_port(&a,i){
            println!("port {} is open on {} ip address",i,a);
        }
    }

}
