# warehouse_packer

Implementing warehouse packer to learn a bit of rust for fun. 

Warehouse Pick Path Optimization Algorithm Analysis
http://www.worldcomp-proceedings.com/proc/p2015/FCS2609.pdf


https://www.erim.eur.nl/material-handling-forum/research-education/tools/calc-order-picking-time/references/#Hall



##the warehouse layout
Representing a warehouse is simply a matter or defining the rows and aisles as a graph. We'll use a simple 4X4 matrix (16 nodes) for most of our testing data. the depot represents the start point for order collection and the end point for order dump off -- imagine you everything everything into a box there. 

We can represent the Nodes (A,B,C,D....) and Vertices (A->B) in a simple data map with the number of steps between them as a weight. e.g. A->B:6, A->E:4. 

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
Q------R-DEPO-S------T

###Map for testing
A->B:6,
B->C:6
C->D:6
E->F:6
F->G:6
G->H:6
I->J:6
J->K:6
K->L:6
M->N:6
N->O:6
O->P:6
Q->R:6
R->DEPO:1
DEPO->S:1
S->T:6
A->E:4
E->I:4
I->M:4
M->Q:1
B->F:4
F->J:4
J->N:4
N->R:1
C->G:4
G->K:4
K->O:4
O->S:1
D->H:4
H->L:4
L->P:4
P->T:1









```

###Storage Bins 
Storage bins contain products. We'll label the bins in the aisle they are in. I'll call the aisles "AE", "BF" to represent a name for them. Within them we;ll say the a storage bin is in a bin that is the index from the row closeest to the DEPOT. e.g. AE.1 (first bin in the AE row.) 