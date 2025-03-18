// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    store: [(Weekday, Option<i32>); 7],
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        Self {
            store: [
                (Weekday::Monday, None),
                (Weekday::Tuesday, None),
                (Weekday::Wednesday, None),
                (Weekday::Thursday, None),
                (Weekday::Friday, None),
                (Weekday::Saturday, None),
                (Weekday::Sunday, None),
            ],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        match day {
            Weekday::Monday => self.store.get(0).and_then(|inner| inner.1),
            Weekday::Tuesday => self.store.get(1).and_then(|inner| inner.1),
            Weekday::Wednesday => self.store.get(2).and_then(|inner| inner.1),
            Weekday::Thursday => self.store.get(3).and_then(|inner| inner.1),
            Weekday::Friday => self.store.get(4).and_then(|inner| inner.1),
            Weekday::Saturday => self.store.get(5).and_then(|inner| inner.1),
            Weekday::Sunday => self.store.get(6).and_then(|inner| inner.1),
        }
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        let val = (day,Some(temperature));
        match val.0 {
            Weekday::Monday => self.store[0] = val,
            Weekday::Tuesday => self.store[1] = val,
            Weekday::Wednesday => self.store[2] = val,
            Weekday::Thursday => self.store[3] = val,
            Weekday::Friday => self.store[4] = val,
            Weekday::Saturday => self.store[5] = val,
            Weekday::Sunday => self.store[6] = val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
