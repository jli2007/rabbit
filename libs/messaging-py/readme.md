RabbitMQ Topology

## exchange
name        : type

agent.swarm : topic

## queues
name                : bound to

classifier_queue    : agent.swarm
investigator_queue  : agent.swarm

## routing keys
key                        : publisher          : consumer

issue.classify             : webhook-receiver   : classifier
issue.investigate.{module} : classifier         : investigator  