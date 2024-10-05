import ipaddress

def calculate_public_ips(subnets, private_ranges):
    available_ips_info = {}
    
    # Convert private ranges to network objects
    excluded_ips = set()
    for private in private_ranges:
        excluded_network = ipaddress.ip_network(private)
        excluded_ips.update(excluded_network.hosts())
    
    # Calcula el total de IPs y las IPs disponibles
    for subnet in subnets:
        network = ipaddress.ip_network(subnet, strict=False)
        total_ips = network.num_addresses - 2  # Exclude network and broadcast
        available_ips = [ip for ip in network.hosts() if ip not in excluded_ips]
        available_count = len(available_ips)
        
        # Save the information
        available_ips_info[subnet] = {
            "total_ips": total_ips,
            "available_count": available_count,
            "available_ips": available_ips
        }
        
        # Print the available IPs
        print(f"\nDirecciones IP disponibles en {subnet}:")
        for ip in available_ips:
            print(ip)
    
    return available_ips_info

# Define public subnets and private ranges
public_subnets = [
    "1.0.0.0/8",
    "2.0.0.0/8",
    "3.0.0.0/8",
    # Pending to add more here
]

private_ranges = [
    "10.0.0.0/8",
    "172.16.0.0/12",
    "192.168.0.0/16",
]

# Calculate the available public IPs
public_ips_result = calculate_public_ips(public_subnets, private_ranges)

