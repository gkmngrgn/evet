import datetime
import typing as t

import click
from evet.date import EventDate
from evet.log import get_logger, set_log_level
from evet.utils import click_coroutine

logger = get_logger()


@click.command()
@click_coroutine
@click.option(
    "-m",
    "--message",
    help="Set event message, title.",
    required=True,
    type=click.STRING,
)
@click.option(
    "-d",
    "--date",
    help="Set event date.",
    required=True,
    type=click.DateTime(formats=["%Y-%m-%d %H:%M"]),
)
@click.option(
    "-l",
    "--local",
    help="Specify the base timezone for the event.",
    type=str,
)
@click.option(
    "-t",
    "--timezone",
    help="Specify the timezone for adding event date by timezone.",
    type=str,
    multiple=True,
)
@click.option(
    "-v",
    "--verbose",
    count=True,
    help="Set verbosity for printing more logging messages.",
)
async def cli(
    message: str,
    date: datetime.datetime,
    local: str,
    timezone: t.Tuple[str],
    verbose: int,
) -> None:
    set_log_level(verbosity=verbose)

    event_date = EventDate(date=date, local=local, timezones=timezone)
    logger.debug(
        "current timezone: %s - %s",
        event_date.get_local_zone(),
        event_date.get_local_zone_name(),
    )

    click.echo("---")
    click.echo(message)
    click.echo()

    for tzname, date_str in event_date.get_dates_by_timezones():
        click.echo(f"{tzname}: {date_str}")

    click.echo("---")


def run() -> None:
    cli()
