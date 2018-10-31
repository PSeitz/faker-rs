use rand::{thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Address <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.address
 */
impl Address {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }

  /**
   * Generates random zipcode from format. If format is not specified, the
   * locale's zip format is used.
   *
   * @method faker.address.zipCode
   * @param {String} format
   */
fn zip_code(&self, format: &str) -> String {
    // if zip format is not specified, use the zip format defined for the locale
    if (format.is_none()) {
      let localeFormat = self.faker.address_postcode();
      if (typeof localeFormat == 'string') {
        format = localeFormat;
      } else {
        format = thread_rng().choose(localeFormat).unwrap();
      }
    }
    return Helpers.replaceSymbols(format);
  }

  /**
   * Generates random zipcode from state abbreviation. If state abbreviation is
   * not specified, a random zip code is generated according to the locale's zip format.
   * Only works for locales with postcode_by_state definition. If a locale does not
   * have a postcode_by_state definition, a random zip code is generated according
   * to the locale's zip format.
   *
   * @method faker.address.zipCodeByState
   * @param {String} state
   */
fn zip_code_by_state(&self, state: &str) -> String {
    let zipRange = self.faker.address_postcode_by_state()[state];
    if (zipRange) {
      return faker.random.number(zipRange);
    }
    return this.zipCode();
  }

  /**
   * Generates a random localized city name. The format string can contain any
   * method provided by faker wrapped in `{{}}`, e.g. `{{name.firstName}}` in
   * order to build the city name.
   *
   * If no format string is provided one of the following is randomly used:
   *
   * * `{{address.cityPrefix}} {{name.firstName}}{{address.citySuffix}}`
   * * `{{address.cityPrefix}} {{name.firstName}}`
   * * `{{name.firstName}}{{address.citySuffix}}`
   * * `{{name.lastName}}{{address.citySuffix}}`
   *
   * @method faker.address.city
   * @param {String} format
   */
fn city(&self, format: &str) -> String {
    let formats = [
      '{{address.cityPrefix}} {{name.firstName}}{{address.citySuffix}}',
      '{{address.cityPrefix}} {{name.firstName}}',
      '{{name.firstName}}{{address.citySuffix}}',
      '{{name.lastName}}{{address.citySuffix}}'
    ];

    if (typeof format !== "number") {
      format = faker.random.number(formats.length - 1);
    }

    return f(formats[format]);

  }

  /**
   * Return a random localized city prefix
   * @method faker.address.cityPrefix
   */
fn city_prefix(&self) -> String {
    return thread_rng().choose(self.faker.address_city_prefix()).unwrap();
  }

  /**
   * Return a random localized city suffix
   *
   * @method faker.address.citySuffix
   */
fn city_suffix(&self) -> String {
    return thread_rng().choose(self.faker.address_city_suffix()).unwrap();
  }

  /**
   * Returns a random localized street name
   *
   * @method faker.address.streetName
   */
fn street_name(&self) -> String {
      let result;
      let suffix = faker.address.streetSuffix();
      if (suffix !== "") {
          suffix = " " + suffix
      }

      switch (faker.random.number(1)) {
      case 0:
          result = faker.name.lastName() + suffix;
          break;
      case 1:
          result = faker.name.firstName() + suffix;
          break;
      }
      return result;
  }

  //
  // TODO: change all these methods that accept a boolean to instead accept an options hash.
  //
  /**
   * Returns a random localized street address
   *
   * @method faker.address.streetAddress
   * @param {Boolean} useFullAddress
   */
fn street_address(&self, useFullAddress: &str) -> String {
      if (useFullAddress == undefined) { useFullAddress = false; }
      let address = "";
      switch (faker.random.number(2)) {
      case 0:
          address = Helpers.replaceSymbolWithNumber("#####") + " " + faker.address.streetName();
          break;
      case 1:
          address = Helpers.replaceSymbolWithNumber("####") +  " " + faker.address.streetName();
          break;
      case 2:
          address = Helpers.replaceSymbolWithNumber("###") + " " + faker.address.streetName();
          break;
      }
      return useFullAddress ? (address + " " + faker.address.secondaryAddress()) : address;
  }

  /**
   * streetSuffix
   *
   * @method faker.address.streetSuffix
   */
fn street_suffix(&self) -> String {
      return thread_rng().choose(self.faker.address_street_suffix()).unwrap();
  }

  /**
   * streetPrefix
   *
   * @method faker.address.streetPrefix
   */
fn street_prefix(&self) -> String {
      return thread_rng().choose(self.faker.address_street_prefix()).unwrap();
  }

  /**
   * secondaryAddress
   *
   * @method faker.address.secondaryAddress
   */
fn secondary_address(&self) -> String {
      return Helpers.replaceSymbolWithNumber(thread_rng().choose(
          [
              'Apt. ###',
              'Suite ###'
          ]
      ));
  }

  /**
   * county
   *
   * @method faker.address.county
   */
fn county(&self) -> String {
    return thread_rng().choose(self.faker.address_county()).unwrap();
  }

  /**
   * country
   *
   * @method faker.address.country
   */
fn country(&self) -> String {
    return thread_rng().choose(self.faker.address_country()).unwrap();
  }

  /**
   * countryCode
   *
   * @method faker.address.countryCode
   */
fn country_code(&self) -> String {
    return thread_rng().choose(self.faker.address_country_code()).unwrap();
  }

  /**
   * state
   *
   * @method faker.address.state
   * @param {Boolean} useAbbr
   */
fn state(&self, useAbbr: &str) -> String {
      return thread_rng().choose(self.faker.address_state()).unwrap();
  }

  /**
   * stateAbbr
   *
   * @method faker.address.stateAbbr
   */
fn state_abbr(&self) -> String {
      return thread_rng().choose(self.faker.address_state_abbr()).unwrap();
  }

  /**
   * latitude
   *
   * @method faker.address.latitude
   * @param {Double} max default is 90
   * @param {Double} min default is -90
   * @param {number} precision default is 4
   */
fn latitude(&self, max: &str,  min: &str,  precision: &str) -> String {
      max       = max || 90
      min       = min || -90
      precision = precision || 4

      return faker.random.number({
        max: max,
        min: min,
        precision: parseFloat((0.0).toPrecision(precision) + '1')
      }).toFixed(precision);
  }

  /**
   * longitude
   *
   * @method faker.address.longitude
   * @param {Double} max default is 180
   * @param {Double} min default is -180
   * @param {number} precision default is 4
   */
fn longitude(&self, max: &str,  min: &str,  precision: &str) -> String {
      max       = max || 180
      min       = min || -180
      precision = precision || 4

      return faker.random.number({
        max: max,
        min: min,
        precision: parseFloat((0.0).toPrecision(precision) + '1')
      }).toFixed(precision);
  }

  /**
   *  direction
   *
   * @method faker.address.direction
   * @param {Boolean} useAbbr return direction abbreviation. defaults to false
   */
fn direction(&self, useAbbr: &str) -> String {
    if (useAbbr.is_none() || useAbbr == false) {
      return thread_rng().choose(self.faker.address_direction()).unwrap();
    }
    return thread_rng().choose(self.faker.address_direction_abbr()).unwrap();
  }

  this.direction.schema = {
    "description": "Generates a direction. Use optional useAbbr bool to return abbrevation",
    "sampleResults": ["Northwest", "South", "SW", "E"]
  };

  /**
   * cardinal direction
   *
   * @method faker.address.cardinalDirection
   * @param {Boolean} useAbbr return direction abbreviation. defaults to false
   */
fn cardinal_direction(&self, useAbbr: &str) -> String {
    if (useAbbr.is_none() || useAbbr == false) {
      return (
        thread_rng().choose(self.faker.address_direction_slice()(0, 4)).unwrap()
      );
    }
    return (
      thread_rng().choose(self.faker.address_direction_abbr_slice()(0, 4)).unwrap()
    );
  }

  this.cardinalDirection.schema = {
    "description": "Generates a cardinal direction. Use optional useAbbr boolean to return abbrevation",
    "sampleResults": ["North", "South", "E", "W"]
  };

  /**
   * ordinal direction
   *
   * @method faker.address.ordinalDirection
   * @param {Boolean} useAbbr return direction abbreviation. defaults to false
   */
fn ordinal_direction(&self, useAbbr: &str) -> String {
    if (useAbbr.is_none() || useAbbr == false) {
      return (
        thread_rng().choose(self.faker.address_direction_slice()(4, 8)).unwrap()
      );
    }
    return (
      thread_rng().choose(self.faker.address_direction_abbr_slice()(4, 8)).unwrap()
    );
  }

  this.ordinalDirection.schema = {
    "description": "Generates an ordinal direction. Use optional useAbbr boolean to return abbrevation",
    "sampleResults": ["Northwest", "Southeast", "SW", "NE"]
  };

fn nearby_gps_coordinate(&self, coordinate: &str,  radius: &str,  isMetric: &str) -> String {
        function randomFloat(min, max) {
            return Math.random() * (max-min) + min;
        }
        function degreesToRadians(degrees) {
            return degrees * (Math.PI/180.0);
        }
        function radiansToDegrees(radians) {
            return radians * (180.0/Math.PI);
        }
        function kilometersToMiles(miles) {
            return miles * 0.621371;
        }
        function coordinateWithOffset(coordinate, bearing, distance, isMetric) {
            let R = 6378.137; // Radius of the Earth (http://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html)
            let d = isMetric ? distance : kilometersToMiles(distance); // Distance in km

            let lat1 = degreesToRadians(coordinate[0]); //Current lat point converted to radians
            let lon1 = degreesToRadians(coordinate[1]); //Current long point converted to radians

            let lat2 = Math.asin(Math.sin(lat1) * Math.cos(d/R) +
                Math.cos(lat1) * Math.sin(d/R) * Math.cos(bearing));

            let lon2 = lon1 + Math.atan2(
                Math.sin(bearing) * Math.sin(d/R) * Math.cos(lat1),
                Math.cos(d/R) - Math.sin(lat1) * Math.sin(lat2));

            // Keep longitude in range [-180, 180]
            if (lon2 > degreesToRadians(180)) {
                lon2 = lon2 - degreesToRadians(360);
            } else if (lon2 < degreesToRadians(-180)) {
                lon2 = lon2 + degreesToRadians(360);
            }

            return [radiansToDegrees(lat2), radiansToDegrees(lon2)];
        }

        // If there is no coordinate, the best we can do is return a random GPS coordinate.
        if (coordinate == undefined) {
            return [this.latitude(), this.longitude()]
        }
        radius = radius || 10.0;
        isMetric = isMetric || false;

        // TODO: implement either a gaussian/uniform distribution of points in cicular region.
        // Possibly include param to function that allows user to choose between distributions.

        // This approach will likely result in a higher density of points near the center.
        let randomCoord = coordinateWithOffset(coordinate, degreesToRadians(Math.random() * 360.0), radius, isMetric);
        return [randomCoord[0].toFixed(4), randomCoord[1].toFixed(4)];
    }

  return this;
}

module.exports = Address;
