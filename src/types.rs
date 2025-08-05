//! Rust Types.

use std::convert::From;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::pg::Pg;
use postgis::ewkb::Point;
use crate::sql_types::*;
use diesel::pg::PgValue;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(sql_type = Geography)]
pub struct GeogPoint {
	pub x: f64, // lon
	pub y: f64, // lat
	pub srid: Option<i32>,
}

impl From<Point> for GeogPoint {
	fn from(p: Point) -> Self {
		let Point { x, y, srid } = p;
		Self { x, y, srid }
	}
}
impl From<GeogPoint> for Point {
	fn from(p: GeogPoint) -> Self {
		let GeogPoint { x, y, srid } = p;
		Self { x, y, srid }
	}
}

impl FromSql<Geography, Pg> for GeogPoint {
	fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
		let bytes = bytes.as_bytes();
		if bytes.len() <= 0 {
			return Err("Received less than 1 bytes while decoding a GeogPoint".into());
		}
		use std::io::Cursor;
		use postgis::ewkb::EwkbRead;
		//let bytes = not_none!(bytes);
		let mut rdr = Cursor::new(bytes);
		Ok(Point::read_ewkb(&mut rdr)?.into())
	}
}

impl ToSql<Geography, Pg> for GeogPoint {
	fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
		use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
		Point::from(*self).as_ewkb().write_ewkb(out)?;
		Ok(IsNull::No)
	}
}
