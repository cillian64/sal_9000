# sal_9000
A new discord bot for the Rebellious guild

## Installation
* Put your discord token in a file called `sal_9000/discord_token.txt`
  * `echo my_token_here >sal_9000/discord_token.txt`
  * `chmod go-rwx sal_9000/discord_token.txt`
* Edit `sal9000.service` and set the username and working directory
* Install the systemd service:
  * `sudo cp sal9000.service /etc/systemd/system`
  * `sudo chown root:root /etc/systemd/system/sal9000.service`
  * `sudo systemd enable sal9000`
  * `sudo systemd start sal9000`
