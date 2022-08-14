# Circular Queue, (Ring Buffer)
A queue in general is a (FIFO) data structure.
- FIFO = First In, First Out

A queue implementation that fixes a common drawback of a regular queue.

#### Regular Queue
``` regular-queue
[1, 2, 3, 4]
 ^        ^
head     tail

de-queue
[*, 2, 3, 4]
    ^     ^
   head  tail

queue is full; can't enqueue
[*, 2, 3, 4]
    ^     ^
   head  tail
```

#### Circular Queue
``` circular-queue
[1, 2, 3, 4]
 ^        ^
head     tail

de-queue
[*, 2, 3, 4]
    ^     ^
   head  tail

en-queue
[5, 2, 3, 4]
 ^  ^     
tail head
```
