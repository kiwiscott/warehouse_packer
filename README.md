# warehouse_packer

Implementing warehouse packer to learn a bit of rust for fun. 

Warehouse Pick Path Optimization Algorithm Analysis
http://www.worldcomp-proceedings.com/proc/p2015/FCS2609.pdf


https://www.erim.eur.nl/material-handling-forum/research-education/tools/calc-order-picking-time/references/#Hall



##the warehouse 
Representing a warehouse is simply a matter or defining the rows and aisles as a graph. We'll use a simple 4X4 matrix (16 nodes) for most of our testing data. the depot represents the start point for order collection and the end point for order dump off -- imagine you out everything into a box there. 

```
A------B------C------D
|      |      |      |
|      |      |      |
|      |      |      |
X      |      |      |
E------F------G------H
|      |      |      |
|      |      |      |
|      |      |      |
|      |      |      |
I------J------K------L
|      |      |      |
|      |      |      |
|      |      |      |
|      |      |      |
M------N------O------P
|      |      |      |
|------DEPOT---------|
```

###Storage Bins 
Storage bins contain products. We'll label the bins in the aisle they are in. I'll call the aisles "AE", "BF" to represent a name for them. Within them we;ll say the a storage bin is in a bin that is the index from the row closeest to the DEPOT. e.g. AE.1 (first bin in the AE row.) 