

#[allow(dead_code)]
use neon::prelude::*;
use std::str::FromStr;
use ip_network::{ Ipv4Network, Ipv6Network };
use cidr::{ AnyIpCidr, Family };

// #[macro_use]
// extern crate neon;
// #[macro_use]
// extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Data {
    _subnet: String,
    _prefix: u8,
    _subnets: Vec<String>
}

fn handle_ipv4_allocation(_subnet: &str, _prefix: u8, _subnets: Vec<String>) -> String {
    let mut i = 0;
    let mut available_subnet = vec![];
    let root_subnet = Ipv4Network::from_str(_subnet).unwrap();
    let allocated_list = _subnets.clone().into_iter();
    let mut allocated_subnets = vec![];
    let subnets_list = root_subnet.subnets_with_prefix(_prefix);
    // let mut left_vector = vec![];
    // let mut right_vector = vec![];
    // println!("Subnet: {:?}", root_subnet);
    // println!("Prefix: {}", _prefix);
    for val in allocated_list {
        i += 1;
        // let mut cloned_subnets = root_subnet.clone().subnets_with_prefix(_prefix);
        let allocated_network = Ipv4Network::from_str(&val);
        // let failed_list = cloned_subnets.find(|x| x.contains(allocated_network.as_ref().unwrap().network_address())).into_iter();
        // for item in failed_list {
        //     i += 1;
        //     let tester = left_vector.iter().any(|v| v == &item);
        //     println!("Allocated list item: {}", val);
        //     println!("Tester: {}", tester);
        //     if !tester {
        //         left_vector.push(item);
        //     } 
        // }
        allocated_subnets.push(allocated_network.unwrap());
    }
  
    for item in subnets_list {
        i +=1;
        let cloned_allocated = &allocated_subnets.clone();
        let test1 = item.network_address();
        // let testF = cloned_allocated.iter().any(|v| v == &item.broadcast_address().);
        let test = cloned_allocated.into_iter().find(|x| x.contains(test1));
        if let None = test  {
            if available_subnet.len() == 0{ 
                available_subnet.push(item) 
            }
            break;
        }
        // for val in test{
        //     i += 1;
        //     println!("Allocated list item: {:?}", val);

        // }
        // let valid_subnet = left_vector.iter().any(|v| v == &item);
        // if !valid_subnet && right_vector.len() <= 15 {
        //     right_vector.push(item);
        // }
    }




    let res = available_subnet[0].to_string();
    println!("Success: {:?}, loops: {}", res, i);
    return String::from(res);
}

fn handle_ipv6_allocation(_subnet: &str, _prefix: u8, _subnets: Vec<String>) -> String {
    let mut i = 0;
    let mut available_subnet = vec![];
    let root_subnet = Ipv6Network::from_str(_subnet).unwrap();
    let allocated_list = _subnets.clone().into_iter();
    let mut allocated_subnets = vec![];
    let subnets_list = root_subnet.subnets_with_prefix(_prefix);
    // let mut left_vector = vec![];
    // let mut right_vector = vec![];
    // println!("Subnet: {:?}", root_subnet);
    // println!("Prefix: {}", _prefix);
    for val in allocated_list {
        i += 1;
        // let mut cloned_subnets = root_subnet.clone().subnets_with_prefix(_prefix);
        let allocated_network = Ipv6Network::from_str(&val);
        // let failed_list = cloned_subnets.find(|x| x.contains(allocated_network.as_ref().unwrap().network_address())).into_iter();
        // for item in failed_list {
        //     i += 1;
        //     let tester = left_vector.iter().any(|v| v == &item);
        //     println!("Allocated list item: {}", val);
        //     println!("Tester: {}", tester);
        //     if !tester {
        //         left_vector.push(item);
        //     } 
        // }
        allocated_subnets.push(allocated_network.unwrap());
    }
  
    for item in subnets_list {
        i +=1;
        let cloned_allocated = &allocated_subnets.clone();
        let test1 = item.network_address();
        // let testF = cloned_allocated.iter().any(|v| v == &item.broadcast_address().);
        let test = cloned_allocated.into_iter().find(|x| x.contains(test1));
        if let None = test  {
            if available_subnet.len() == 0{ 
                available_subnet.push(item) 
            }
            break;
        }
        // for val in test{
        //     i += 1;
        //     println!("Allocated list item: {:?}", val);

        // }
        // let valid_subnet = left_vector.iter().any(|v| v == &item);
        // if !valid_subnet && right_vector.len() <= 15 {
        //     right_vector.push(item);
        // }
    }




    
    let res = available_subnet[0].to_string();
    println!("Success: {:?}, loops: {}", res, i);
    return String::from(res);
}


fn allocate_network_from_subnet(mut cx: FunctionContext) -> JsResult<JsString> {
    // let collection_js = cx.argument::<JsArray>(0)?;
    // let collection_rust: Vec<Handle<JsValue>> = collection_js.to_vec(&mut cx)?;
    // println!("Argument: {:?}", collection_rust);
    let arg0 = cx.argument::<JsValue>(0)?;
    let parsed_input: Data = neon_serde::from_value(&mut cx, arg0)?;
    println!("{:?}", parsed_input);

    // arguments: subnet , prefix , allocated subnets
    // check if subnet is ipV4 or ipv6
    let cidr = parsed_input._subnet;
    let allocated_networks = parsed_input._subnets;
    let prefix_length = parsed_input._prefix;
    let network_family = cidr.parse::<AnyIpCidr>().unwrap().family().unwrap();
    let mut result = String::from("");

    if let Family::Ipv4 = network_family {
        result = handle_ipv4_allocation(&cidr, prefix_length, allocated_networks);
    }
    else if let Family::Ipv6 = network_family {
        result = handle_ipv6_allocation(&cidr, prefix_length, allocated_networks);
    }
   
    Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("allocateNetworkFromSubnet", allocate_network_from_subnet)
});
