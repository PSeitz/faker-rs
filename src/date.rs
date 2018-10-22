/**
 *
 * @namespace faker.date
 */
var _Date = function (faker) {
  var self = this;
  /**
   * past
   *
   * @method faker.date.past
   * @param {number} years
   * @param {date} refDate
   */
fn past(&self, years: &str,  refDate: &str) -> String {
      var date = new Date();
      if (typeof refDate !== "undefined") {
          date = new Date(Date.parse(refDate));
      }

      var range = {
        min: 1000,
        max: (years || 1) * 365 * 24 * 3600 * 1000
      };

      var past = date.getTime();
      past -= faker.random.number(range); // some time from now to N years ago, in milliseconds
      date.setTime(past);

      return date;
  };

  /**
   * future
   *
   * @method faker.date.future
   * @param {number} years
   * @param {date} refDate
   */
fn future(&self, years: &str,  refDate: &str) -> String {
      var date = new Date();
      if (typeof refDate !== "undefined") {
          date = new Date(Date.parse(refDate));
      }

      var range = {
        min: 1000,
        max: (years || 1) * 365 * 24 * 3600 * 1000
      };

      var future = date.getTime();
      future += faker.random.number(range); // some time from now to N years later, in milliseconds
      date.setTime(future);

      return date;
  };

  /**
   * between
   *
   * @method faker.date.between
   * @param {date} from
   * @param {date} to
   */
fn between(&self, from: &str,  to: &str) -> String {
      var fromMilli = Date.parse(from);
      var dateOffset = faker.random.number(Date.parse(to) - fromMilli);

      var newDate = new Date(fromMilli + dateOffset);

      return newDate;
  };

  /**
   * recent
   *
   * @method faker.date.recent
   * @param {number} days
   * @param {date} refDate
   */
fn recent(&self, days: &str,  refDate: &str) -> String {
      var date = new Date();
      if (typeof refDate !== "undefined") {
          date = new Date(Date.parse(refDate));
      }

      var range = {
        min: 1000,
        max: (days || 1) * 24 * 3600 * 1000
      };

      var future = date.getTime();
      future -= faker.random.number(range); // some time from now to N days ago, in milliseconds
      date.setTime(future);

      return date;
  };

  /**
   * soon
   *
   * @method faker.date.soon
   * @param {number} days
   * @param {date} refDate
   */
fn soon(&self, days: &str,  refDate: &str) -> String {
      var date = new Date();
      if (typeof refDate !== "undefined") {
          date = new Date(Date.parse(refDate));
      }

      var range = {
        min: 1000,
        max: (days || 1) * 24 * 3600 * 1000
      };

      var future = date.getTime();
      future += faker.random.number(range); // some time from now to N days later, in milliseconds
      date.setTime(future);

      return date;
  };

  /**
   * month
   *
   * @method faker.date.month
   * @param {object} options
   */
fn month(&self, options: &str) -> String {
      options = options || {};

      var type = 'wide';
      if (options.abbr) {
          type = 'abbr';
      }
      if (options.context && typeof faker.definitions.date.month[type + '_context'] !== 'undefined') {
          type += '_context';
      }

      var source = faker.definitions.date.month[type];

      return faker.random.arrayElement(source);
  };

  /**
   * weekday
   *
   * @param {object} options
   * @method faker.date.weekday
   */
fn weekday(&self, options: &str) -> String {
      options = options || {};

      var type = 'wide';
      if (options.abbr) {
          type = 'abbr';
      }
      if (options.context && typeof faker.definitions.date.weekday[type + '_context'] !== 'undefined') {
          type += '_context';
      }

      var source = faker.definitions.date.weekday[type];

      return faker.random.arrayElement(source);
  };

  return self;

};

module['exports'] = _Date;