import os
from asyncio import sleep
import ipaddress
import requests
import time

# Function to generate IP ranges dynamically from start to end
def generate_ip_ranges(start, end) -> list:
    ip_ranges = []
    for i in range(start, end + 1):
        ip_ranges.append(f"{i}.0.0.0/8")
    return ip_ranges

# Function to calculate public IPs excluding private ranges
def calculate_public_ips(subnets, private_ranges, output_dir):
    available_ips_info = {}

    # Convert private ranges to network objects
    excluded_ips = set()
    for private in private_ranges:
        excluded_network = ipaddress.ip_network(private)
        excluded_ips.update(excluded_network.hosts())

    # Ensure the output directory exists
    os.makedirs(output_dir, exist_ok=True)

    # Calculate total IPs and available IPs
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

        # Print and write the available IPs to a file only if the file doesn't exist
        print(f"\nAvailable IPs in subnet: {subnet}:")
        net = f"{subnet}"
        file_name = f"{output_dir}/ip_list_{net.split('/')[0]}.txt"

        if not os.path.exists(file_name):
            # Writing to file
            with open(file_name, "a+") as file1:
                for ip in available_ips:
                    file1.write(str(ip) + "\n")
        else:
            print(f"File {file_name} already exists. Skipping write.")

    # return available_ips_info

# Define private ranges (will be excluded)
private_ranges = [
    "10.0.0.0/8",
    "172.16.0.0/12",
    "192.168.0.0/16",
]

def main() -> None:
    # Generate public IP ranges from 1.0.0.0/8 to 223.0.0.0/8
    public_subnets = generate_ip_ranges(1, 223)
    # public_subnets = ["13.0.0.0/8"]

    # Directory to store the IP list files
    output_directory = "/mnt/ssd/ip_list"

    # Calculate the available public IPs excluding private ranges
    print(f"Calculating IPs for {len(public_subnets)} subnets")
    calculate_public_ips(public_subnets, private_ranges, output_directory)

if __name__ == "__main__":
    main()
