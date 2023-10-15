# The Configuration Module

This module gets intiialized by ```Configuration::new()```.
The module will then attempt to read the relative file ```Configuration.json``` file.

## API
- ```Configuration::new()``` creates a new instance of the configuration module.
- ```load(): Option<ConicalError>``` loads the configuration into memory. This will also create the configuration file if it does not already exist.
