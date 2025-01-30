CREATE TABLE bookings (
                          id UUID PRIMARY KEY,
                          property_id UUID NOT NULL,
                          user_id UUID NOT NULL,
                          start_date DATE NOT NULL,
                          end_date DATE NOT NULL,
                          status TEXT NOT NULL
);