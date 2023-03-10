Tripolys
====================================

A python program for checking homomorphisms and testing polymorphism conditions of
directed graphs. 

This repository contains companion code for the following paper. If you use this
code, please cite the paper. You can use the bibtex reference below. (TODO
update once published)

_M. Bodirsky, J. Bulín, F. Starke, and M. Wernthaler. The smallest hard trees, arXiv:2205.07528 [math.RA] (May 2022)_
https://doi.org/10.48550/arXiv.2205.07528
 
```
@misc{https://doi.org/10.48550/arxiv.2205.07528,
  doi = {10.48550/ARXIV.2205.07528},
  url = {https://arxiv.org/abs/2205.07528},
  author = {Bodirsky, Manuel and Bulín, Jakub and Starke, Florian and Wernthaler, Michael},  
  keywords = {Rings and Algebras (math.RA), FOS: Mathematics, FOS: Mathematics, G.2.2, 08A70, 08B05},  
  title = {The Smallest Hard Trees},
  publisher = {arXiv},
  year = {2022},
  copyright = {Creative Commons Attribution Non Commercial Share Alike 4.0 International}
}
```

Introduction
-----------------
In the paper *The Smallest Hard Trees*, we study computational and descriptive
complexity of fixed-template CSPs for small orientations of trees. The paper
contains a number of experimental results (see Section 7). Below you can find
the commands to reproduce those results. Edgelists of all trees with up to 20 vertices can be found [here](https://gitlab.com/WhatDothLife/tripolys_data/-/tree/master/).


Usage
-----------------

Example to find all trees in a given file that do not have totally symmetric polymorphisms:
```
import treeGeneration
Ts = treeGeneration.getTreesFromFile('name of file with trees')
TsNoAC = [T for T in Ts if not treeGeneration.isTotallySymmetric(T)]
```



Contact
-----------------
You can report issues and ask questions in the repository's issues page. 

License
-----------------
This program is released under the terms of the GNU General Public License v3.0.

Visit this [page](http://gnugpl.org/) for license details.
