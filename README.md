# [Advent of Code 2023](https://adventofcode.com/2023)

## Usage

To make it simply to use for everyone without needs to install any dependencies this repo is prepared to test everything with docker container.

* Run `make build` to build docker image with correct user permissions
* Run `make up` to start docker container
* Run `make ssh` to exec into container and run solutions

## Solutions

To run and check solution after exec into container you need to use some CLI commands inside `/mnt/app` directory:

* Use `cargo run day1` to solve Day 1 solutions ([Trebuchet?!](https://adventofcode.com/2023/day/1))
