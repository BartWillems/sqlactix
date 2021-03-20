just a small PoC to test <https://github.com/launchbadge/sqlx>

Features to test:

* good db error conversions (eg: dubplicate insert becomes http 409 conflict)
* tokio tracing
* "complex" queries like joins etc
* state information like busy connection count
* migration management (how to run them, rolling back, etc)