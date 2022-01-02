
# How Monitoring is implemented

A brief description of how Monitoring is implemented

## CPU Monitor


* cpu monitor starts by measuring the current cpu usage.
* then it loops through all items mentioen in conifg json checking what targets is it crossing.
* At the end of above loop we get all the items where target is being breached
* After this we find the maximum value of target being breached.
* After this notification is send only for max value of target breached


### Screenshots
![App Screenshot](https://raw.githubusercontent.com/OutOfBoundCats/monitor/dev/documentation/images/cpu_digram.png)



