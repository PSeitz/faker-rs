use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
/**
 *
 * @namespace faker.date
 */
let _Date = function (faker) {
  /**
   * past
   *
   * @method faker.date.past
   * @param {number} years
   * @param {date} refDate
   */
    pub fn past(&self, years: &str,  refDate: &str) -> String {
      let date = new Date();
      if (refDate.is_some()) {
          date = new Date(Date.parse(refDate));
      }

      let range = {
        min: 1000,
        max: (years || 1) * 365 * 24 * 3600 * 1000
      };

      let past = date.getTime();
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
    pub fn future(&self, years: &str,  refDate: &str) -> String {
      let date = new Date();
      if (refDate.is_some()) {
          date = new Date(Date.parse(refDate));
      }

      let range = {
        min: 1000,
        max: (years || 1) * 365 * 24 * 3600 * 1000
      };

      let future = date.getTime();
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
    pub fn between(&self, from: &str,  to: &str) -> String {
      let fromMilli = Date.parse(from);
      let dateOffset = faker.random.number(Date.parse(to) - fromMilli);

      let newDate = new Date(fromMilli + dateOffset);

      return newDate;
  };

  /**
   * recent
   *
   * @method faker.date.recent
   * @param {number} days
   * @param {date} refDate
   */
    pub fn recent(&self, days: &str,  refDate: &str) -> String {
      let date = new Date();
      if (refDate.is_some()) {
          date = new Date(Date.parse(refDate));
      }

      let range = {
        min: 1000,
        max: (days || 1) * 24 * 3600 * 1000
      };

      let future = date.getTime();
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
    pub fn soon(&self, days: &str,  refDate: &str) -> String {
      let date = new Date();
      if (refDate.is_some()) {
          date = new Date(Date.parse(refDate));
      }

      let range = {
        min: 1000,
        max: (days || 1) * 24 * 3600 * 1000
      };

      let future = date.getTime();
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
    pub fn month(&self, options: &str) -> String {
      options = options || {};

      let type = 'wide';
      if (options.abbr) {
          type = 'abbr';
      }
      if (options.context && typeof self.faker.date_month()[type + '_context'] !== 'undefined') {
          type += '_context';
      }

      let source = self.faker.date_month()[type];

      return thread_rng().choose(source).unwrap();
  };

  /**
   * weekday
   *
   * @param {object} options
   * @method faker.date.weekday
   */
    pub fn weekday(&self, options: &str) -> String {
      options = options || {};

      let type = 'wide';
      if (options.abbr) {
          type = 'abbr';
      }
      if (options.context && typeof self.faker.date_weekday()[type + '_context'] !== 'undefined') {
          type += '_context';
      }

      let source = self.faker.date_weekday()[type];

      return thread_rng().choose(source).unwrap();
  };

  return self;

};

module['exports'] = _Date;
