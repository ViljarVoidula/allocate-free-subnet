

#[allow(dead_code)]
use neon::prelude::*;
use std::str::FromStr;
use ip_network::{ Ipv4Network, Ipv6Network };
use cidr::{ AnyIpCidr, Family };


extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Data {
    _subnet: String,
    _prefix: u8,
    _subnets: Vec<String>
}

pub fn handle_ipv4_allocation(_subnet: &str, _prefix: u8, _subnets: Vec<String>) -> String {
    debug!("Loading ipv4 allocation function");
    let mut available_subnet = vec![];
    debug!("#parse subnet");
    let root_subnet = Ipv4Network::from_str(_subnet).unwrap();
    debug!("parse subnet: {:?}", root_subnet);
    let allocated_list = _subnets.iter();
    let mut allocated_subnets = vec![];
    let subnets_list = root_subnet.subnets_with_prefix(_prefix);
    debug!("Subnet: {:?}", root_subnet);
    debug!("Prefix: {}", _prefix);
    for val in allocated_list {
        // let mut cloned_subnets = root_subnet.clone().subnets_with_prefix(_prefix);
        let allocated_network = Ipv4Network::from_str(&val);
        debug!("Allocated network: {:?}", allocated_network);
        allocated_subnets.push(allocated_network.unwrap());
    }
  
    for item in subnets_list {
        let cloned_allocated = &allocated_subnets.clone();
        let test = cloned_allocated.iter().find(|x| x.contains(item.network_address()));
        if let None = test  {
            if available_subnet.len() == 0{ 
                available_subnet.push(item) 
            }
            break;
        }
    }
    return String::from(available_subnet[0].to_string());
}

pub fn handle_ipv6_allocation(_subnet: &str, _prefix: u8, _subnets: Vec<String>) -> String {
    let mut available_subnet = vec![];
    let root_subnet = Ipv6Network::from_str(_subnet).unwrap();
    let allocated_list = _subnets.iter();
    let mut allocated_subnets = vec![];
    let subnets_list = root_subnet.subnets_with_prefix(_prefix);

    debug!("Subnet: {:?}", root_subnet);
    debug!("Prefix: {}", _prefix);
    for val in allocated_list {
        let allocated_network = Ipv6Network::from_str(&val);
        allocated_subnets.push(allocated_network.unwrap());
    }
  
    for item in subnets_list {
        let root_contains = allocated_subnets
        .iter()  // Vec.iter() is the same as &Vec.into_iter()
        .find(|x| {
            x.contains(item.network_address())
        });
        if let None = root_contains  {
            if available_subnet.len() == 0{ 
                available_subnet.push(item) 
            }
            break;
        }
    }




    
    let res = available_subnet[0].to_string();
    // debug!("Success: {:?}, loops: {}", res, i);
    return String::from(res);
}


pub fn allocate_network_from_subnet(mut cx: FunctionContext) -> JsResult<JsString> {
    debug!("Start processing context:");
    let arg0 = cx.argument::<JsValue>(0)?;
    let parsed_input: Data = neon_serde::from_value(&mut cx, arg0)?;
    debug!("{:?}", parsed_input);

    // arguments: subnet , prefix , allocated subnets
    // check if subnet is ipV4 or ipv6
    let cidr = parsed_input._subnet;
    let allocated_networks = parsed_input._subnets;
    let prefix_length = parsed_input._prefix;
    debug!("CIDR parse time");
    let network_family = cidr.parse::<AnyIpCidr>().unwrap().family().unwrap();
    debug!("Network family: {:?}", network_family);
    let mut result = String::from("");

    if let Family::Ipv4 = network_family {
        debug!("Start processing networks");
        result = handle_ipv4_allocation(&cidr, prefix_length, allocated_networks);
    }
    else if let Family::Ipv6 = network_family {
        result = handle_ipv6_allocation(&cidr, prefix_length, allocated_networks);
    }
   
   return Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("allocateNetworkFromSubnet", allocate_network_from_subnet)
});
