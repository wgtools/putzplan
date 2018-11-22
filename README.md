# putzplan
 A dead simple rust rocket app to host our cleaning plan.
 
 It schedules the periodic tasks occuring in a flat based on defined conditions and assigns them to predefined people.
 
 The resulting table can be accessed via a simple webinterface.

## Building
Make sure you have a working rust toolchain, refer to the rocket docs.
Run `make` to build a Zip-file deployable to a server

## Running
Select your profile with `ROCKET_ENV=production` and run the binary.

For best security - you want to keep it secret when somebody has to clean the kitchen -  run a reverseproxy with authentication and TLS in front of the Rocket server.
