#!/bin/bash
aws s3 mb s3://my-makefile-visualizer
aws s3 sync web/public s3://my-makefile-visualizer --acl public-read
cargo build --release
scp target/release/cloud-makefile-visualizer user@my-ec2-instance:/path/to/deploy
ssh user@my-ec2-instance "nohup ./cloud-makefile-visualizer &"
