
# How Monitoring is implemented

A brief description of how Monitoring is implemented

## CPU Monitor


* cpu monitor starts by measuring the current cpu usage.
* then it loops through all items mentioen in conifg json checking what targets is it crossing.
* At the end of above loop we get all the items where target is being breached
* After this we find the maximum value of target being breached.
* After this notification is send only for max value of target breached

### All the monitoring is done on single thread

### Screenshots
![App Screenshot](https://raw.githubusercontent.com/OutOfBoundCats/monitor/dev/documentation/images/cpu_digram.png)



## Memory Monitor also worrks the same way as CPU monitor


## Ping/services,volume Monitor

* program goes thourgh all the items and instantiates unique thread for monitoring thoes items.

### Screenshots
![App Screenshot](https://raw.githubusercontent.com/OutOfBoundCats/monitor/dev/documentation/images/remaining_monitors.png)