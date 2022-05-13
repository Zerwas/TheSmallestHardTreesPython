Tripolys
====================================

A program for checking homo-/polymorphisms of graphs.

This repository contains companion code for the following paper: TODO

M. Bodirsky, J. Bulín, F. Starke, M. Wernthaler. The smallest hard trees 
<!-- *Statistics and Computing* **32**, 17 (2022). -->

If you use this code please cite the paper using the bibtex reference below. TODO

Introduction
-----------------
In the paper *The Smallest Hard Trees* we propose... TODO

Installation
-----------------
This code is compatible with Rust 2021.

```
git clone https://gitlab.com/WhatDothLife/tripolys.git
cd tripolys
cargo build --release
```

Data
-----------------
<!-- The slurm-scripts to reproduce the main experiments are in the subfolders under -->
<!-- "scripts". You may need to modify the path to the data folder via the -->
<!-- argument `data_path`. -->

Usage
-----------------
- Generate all trees with n vertices (see Section 7 and Table 2):

```
  ./tripolys generate -s n -e n
```

- Generate all core trees with n vertices (see Section 7 and Table 2):
  
```
  ./tripolys generate -s n -e n --core
```

7.1.1. The smallest NP-hard trees: 
-----------------
The trees are [here](file:data/20/no_siggers.csv ) and this is how you can test them:

```
./tripolys polymorphism --input cores.edges --output no_2wnu.csv --condition 2wnu --filter deny
./tripolys polymorphism --input no_2wnu.csv --output no_3wnu.csv --condition 3wnu --filter deny
./tripolys polymorphism --input no_3wnu.csv --output no_kmm.csv --condition kmm --filter deny
```

The smallest NP-hard triads: TODO link edge lists here?
```
./tripolys polymorphism --graph 10110000,0101111,100111 --condition kmm
./tripolys polymorphism --graph 10110000,1001111,010111 --condition kmm
```


7.1.2. TODO
-----------------

7.2.1 A Tree not known to be in NL: 
-----------------
The trees not known to be in NL are [here](https://gitlab.com/WhatDothLife/tripolys_data/-/blob/master/16/no_majority.csv) and this is how you can test them:

```
cd data/16
./tripolys polymorphism --input cores.edges --output no_majority.csv --condition majority --filter deny
```

Other examples
-----------------
Use --help

```
./tripolys polymorphism -H 10110000,100111,010111 -c commutative
```
```
./tripolys polymorphism -H graph.csv -c 3-wnu -I
```
```
./tripolys homomorphism -f graph.csv -t t3
```
```
./tripolys homomorphism -f p5 -t c2
```

Contact
-----------------
You can report issues and ask questions in the repository's issues page. 

License
-----------------
This program is released under the terms of the GNU General Public License v3.0.

Visit this [page](http://gnugpl.org/) for license details.


<!-- Acknowledgements -->
<!-- -------------------------- -->
<!-- The work was supported by the Center for Information Services and High -->
<!-- Performance Computing [Zentrum für Informationsdienste und Hochleistungsrechnen -->
<!-- (ZIH)] at TU Dresden which provided its facilities for high throughput -->
<!-- calculations. -->
