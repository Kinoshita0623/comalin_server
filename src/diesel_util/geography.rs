use std::io::Write;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output};
use diesel::types::{FromSql, ToSql};
use postgis::ewkb::Point;
use crate::diesel_util::sql_types::Geography;
use diesel::backend::Backend;

#[sql_type="Geography"]
#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
pub struct GeographyPoint {
    pub x: f64,
    pub y: f64,
    pub srid: Option<i32>
}


impl From<Point> for GeographyPoint {
    fn from(p: Point) -> Self {
        let Point { x,y , srid} = p;
        return Self {
            x,y,srid
        }
    }
}

impl From<GeographyPoint> for Point {
    fn from(p: GeographyPoint) -> Self {
        return Point {
            x: p.x,
            y: p.y,
            srid: p.srid
        }
    }
}

impl FromSql<Geography, Pg> for GeographyPoint {
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        use std::io::Cursor;
        use postgis::ewkb::EwkbRead;
        let bytes = not_none!(bytes);
        let mut rdr = Cursor::new(bytes);
        return Ok(Point::read_ewkb(&mut rdr)?.into());
    }
}

impl ToSql<Geography, Pg> for GeographyPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
        Point::from(*self).as_ewkb().write_ewkb(out)?;
        Ok(IsNull::No)

    }
}

