This project is intended to show ways to use rust-postgres and supplementary libraries.  There is little to no consistency in style across similar procedures because the intent is to show different approaches.

This project is the backend of a basic conference room scheduling system.
It can't do too much but the little it can do is pretty cool:  A user
can schedule a room and check whether preferred timeslots are available.



Step 1:  Create the database

	- familiarize yourself with the contents of the /src/sql directory
	- either run the shell script /src/sql/create_db.sh from within that directory or manually create
	  the database on your own terms (note that db extensions require admin rights)

Step 2:  Seed the database by executing main.rs, using: ``cargo run`` from within the project directory

Step 3:  Run Tests, using ``cargo test``
