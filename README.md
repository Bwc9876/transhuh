
# Transhuh

Simple program to poll a TransLoc vendor API every few seconds and get bus information.
Notifies you if any bus on the route is at a specified "point of interest" (see below).

## Usage

`transhuh -u API_URL -r ROUTE_ID [-p POI]*`

- `-u`: Provide the base API URL (should end with `JSONPRelay.svc`)
- `-r`: Numeric route ID to track, you can get this by looking at the URL after `/routes`
        when you click on a route in the web UI
- `-p`: A point of interest to watch for. The format is `LABEL:LATITUDE,LONGITUDE`. When the bus gets near
        the specified latitude and longitude you will get a desktop notification saying "The bus is at [LABEL]" where label is what you specified before the `:`. You may specify `-p` multiple times to watch more than
        one point of interest.

The program is smart about notifying you, if a bus never leaves the "zone" of the point of interest
it will not notify you at each poll interval.
