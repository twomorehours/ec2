# ec2 reboot helper

## Description
ec2 reboot helper will stop your instance and start your instance to get an new ip which hasn't be blocked by GFW.

## Prerequisite
- make sure you have Rust&Cargo installed
- [aws access key](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/getting-started.html#getting-started-step2)

## Usage
```shell
➜ ~ git clone github.com/twomorehours/ec2.git
➜ ~ cd ec2
➜ ec2 git:(master) cargo install --path .
➜ ~ ec2 reboot your-instance-id
```
