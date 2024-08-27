# Clonk - a cross-platform system clock
Inspired by the `clock` demo app in chapter 9 of *Rust In Action* by Tim McNamara
## Usage
```shell
clonk [OPTIONS] <action> [datetime]

Arguments:
  <action>    [default: get] [possible values: get, set]
  [datetime]  When <action> is 'set', apply <datetime>. Otherwise, ignore

Options:
  -s, --standard <std>  [default: rfc3339] [possible values: rfc2822, rfc3339, timestamp]
  -h, --help            Print help
  -V, --version         Print version
```
For example,
```shell
$ clonk get
2024-06-06T16:42:55.539188586+00:00
# Set system time (may need to run as superuser)
$ sudo clonk set '2024-01-01T00:00:00.000000000+00:00'
2024-01-01T00:00:00.000886100+00:00
$ clonk get
2024-01-01T00:00:00.776596474+00:00
```
Changing system time can cause problems, like SSL certs being erroneously rejected as expired. I recommend running the `set` command in a virtual machine - see Appendix B below.
# Appendix
## A. About NTP
Two Modes: *always on* and *request/response*.
Always on: multiple computers work in p2p fashion to converge on an agreed definition of now.
- requires a daemon or service, but can achieve tight synchronization

Request/Response: local clients request the time via a single message, then parse the response, keeping track of the elapsed time.
Client compares original timestamp with timestamp sent from server, adjusting for delays from net latency, and adjust local clock towards server's time.

## B. Setting up a VM to Change System Time Safely using Vagrant
### Initialize a Vagrantfile
```shell
# Use Ubuntu 22.04 image from HashiCorp Vagrant Cloud
vagrant init bento/ubuntu-22.04
```
### Configure network
For this demo, use private network 
```ruby
config.vm.network "private_network", ip: "192.168.56.10"
```
### Optional: add a symlink for convenience
This adds a symlink so that the executable is on the `PATH` of the default user `vagrant` as well as `root` on the VM.
```ruby
# In aninline shell provision block
config.vm.provision "shell", inline: <<-SHELL
    ln -s /home/vagrant/app/clonk /usr/bin/clonk
SHELL
```


