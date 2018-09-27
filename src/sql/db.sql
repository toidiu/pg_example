CREATE EXTENSION IF NOT EXISTS "btree_gist";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;


CREATE TABLE testing.users (
	id  BIGSERIAL PRIMARY KEY,
	ext_id UUID NOT NULL DEFAULT  uuid_generate_v4() UNIQUE,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (first_name, last_name)
);


CREATE TABLE testing.building (
	id  BIGSERIAL PRIMARY KEY,
	ext_id UUID NOT NULL DEFAULT  uuid_generate_v4() UNIQUE,
	name   VARCHAR(200) UNIQUE
);


CREATE TABLE testing.room (
	id  BIGSERIAL PRIMARY KEY,
	ext_id UUID NOT NULL DEFAULT  uuid_generate_v4() UNIQUE,
	building_id   BIGINT REFERENCES testing.building(id) NOT NULL,
	code  VARCHAR(10) NOT NULL,
	floor_num   INTEGER NOT NULL,
	UNIQUE(building_id, code) 
);


CREATE TABLE testing.meeting (
	id  BIGSERIAL PRIMARY KEY,
	ext_id UUID NOT NULL DEFAULT  uuid_generate_v4() UNIQUE,
	organizer_id BIGINT REFERENCES testing.users(id) NOT NULL,
	room_id  BIGINT REFERENCES testing.room(id) NOT NULL,
	title   VARCHAR(200) NOT NULL,
	time_slot   TSTZRANGE NOT NULL,
	CONSTRAINT mtg_timeslot_overlap EXCLUDE USING gist (room_id WITH =, time_slot WITH &&)
);
