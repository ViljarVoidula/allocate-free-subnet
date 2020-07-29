var addon = require('../native');
console.time('stat')
console.log(addon.allocateNetworkFromSubnet( { _subnet: "2001:db8::/32", _prefix: 64, _subnets: ["2001:db8::/48", "2001:db8:1::/48", "2001:db8:2::/48"] }));
console.timeEnd('stat')
