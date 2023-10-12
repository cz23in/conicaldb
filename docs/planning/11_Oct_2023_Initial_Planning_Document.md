# Initial Planning Document

| Date | Author | Change |
|---|---|---|
| 12/Oct/2023 | cz23in | Created document |

## Objective

The project will have the following inital objectives:

- Be a relational vertically-scalable database with automatic failover. The situation in mind is to have three servers. Any of these three servers can receive requests and can change the common data set. Through some communication scheme between these three nodes, there is consensus on what the common data set is. If any node goes down, then the data is still avaliable and can be queried through the other two nodes.
- The database should be secured by default. When using SSH, a user simply connects to a server and confirms the fingerprint of the public key. From there the user can communicate securely with the server. In instances of website where users are anonymous and often one-time, certificates make sense as there is no need for priror communication. However, given that users of a database connect often and can have prior connection information, it only makes management more difficult to handle encryption through TLS certificates. Instead, every could have its own encryption key to communicate with the server. Handling encryption in this way makes the management-overhead a lot simpler. Additionally, the database can be secure by default in this way.
- Data for the server should be encrypted at rest and in transit.

Distributed systems are not easy, and choosing a good storage mechanism for the data will ensure fast access. So, some experiments will be done before active development starts:

- Try setup 3 servers and have a master-election process together with a re-election should one of the nodes come offline.
- Try setup 3 servers and have a record sent to a server. Then, on command, kill one of the servers. The data should still exist and should still be queryable.
- Try setup 3 servers and have a node-joining process.
- Write a file format which accepts a basic schema and contains a number of rows. This should have CRUD functionality.

## Initial Timeline

- Work on the intiial experiments.
- Develop a list of binaries/libraries and the associated modules and API interfaces.
- Design a network protocol.
