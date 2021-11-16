import datetime
import typing as t

import click

from evet.utils import click_coroutine


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
    "-t",
    "--timezone",
    help="Specify the timezone for adding event date by timezone.",
    type=str,
)
async def cli(message: str, date: datetime.datetime, timezone: t.List[str]) -> None:
    click.echo(message)
    click.echo(date.strftime("%Y-%m-%d %H:%M"))


def run() -> None:
    cli()
