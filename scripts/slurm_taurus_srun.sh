#!/bin/bash

#SBATCH --time=24:00:00   # walltime
#SBATCH --nodes=1   # number of nodes
#SBATCH --ntasks=1      # limit to one node
#SBATCH --cpus-per-task=24  # number of processor cores (i.e. threads)
#SBATCH --partition=haswell
#SBATCH --mem-per-cpu=5000M   # memory per CPU core
#SBATCH --output=/scratch/ws/0/s8179597-ws_tripolys/majority.output
#SBATCH -J "majority"   # job name
#SBATCH -A p_coretrees

# srun ../target/release/examples/treenum \
#      --from_file /scratch/ws/0/s8179597-ws_tripolys/treenum_data/trees/20/3wnu_n \
#      --data /scratch/ws/0/s8179597-ws_tripolys/treenum_data \
#      --polymorphism siggers \
#      --idempotent \
#      --out /scratch/ws/0/s8179597-ws_tripolys/treenum_data/trees/20/siggers_idempotent_indicator.csv

srun ../target/release/examples/treenum \
      	--data /scratch/ws/0/s8179597-ws_tripolys/tripolys_data \
	--start 1 \
	--end 20 \
	--trees \
	--core \
	--save \
	--polymorphism majority \
	--out majority.csv
