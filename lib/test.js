var addon = require('../native');
console.time('stat')
console.log(addon.allocateNetworkFromSubnet({_subnet: "172.19.0.0/16" , _prefix: 29 , _subnets: ["172.19.0.0/28","172.19.0.24/30","172.19.0.32/30", "172.19.0.58/32","172.19.0.64/26","172.19.0.128/31","172.19.1.0/29","172.19.1.8/29", "172.19.1.128/30","172.19.1.192/30" ]}));
console.timeEnd('stat')
