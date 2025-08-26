-- Initial database setup for meeseeks-nuntius
-- This file is part of a secure messaging project codename meeseeks-nuntius
-- Copyright (C) 2025  Grant DeFayette

-- Create the database if it doesn't exist (this might not be needed since Docker creates it)
-- But ensure proper permissions and extensions

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create indexes for better performance (these will also be created by migrations)
-- This is just to ensure they exist if needed for initial data

-- You can add any initial data here if needed
-- For example, creating an initial relay key pair could go here
-- But for now, we'll let the application handle this through migrations