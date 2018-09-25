#!/bin/bash

# create_db.sql tries to install btree_gist, requiring admin privileges 


psql -U postgres -h 127.0.0.1 -f create_db_and_user.sql
psql -U test_user -d testing_db -h 127.0.0.1 -f db.sql
