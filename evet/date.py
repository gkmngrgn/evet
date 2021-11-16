import datetime
import typing as t

import pytz
from tzlocal import get_localzone


class EventDate(object):
    def __init__(
        self,
        date: datetime.datetime,
        timezones: t.Tuple[str],
    ) -> None:
        self.__date = date
        self.__local_zone = get_localzone()
        self.__timezones = timezones

    def get_dates_by_timezones(self) -> t.List[t.Tuple[str, str]]:
        result = [
            ("Europe/Berlin", "saaaaat budur."),
        ]
        return result

    def get_local_zone(self) -> t.Optional[str]:
        return self.__local_zone.tzname(self.__date)

    def get_local_zone_name(self) -> str:
        return str(self.__local_zone)
