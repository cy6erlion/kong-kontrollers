use super::DatabaseError;
use crate::data::{Product, ProductDatabaseInput, ID};
use postgres::Client;

/// SQL queries for the products database table
pub struct ProductsSql;

impl ProductsSql {
    /// Create a new row
    pub fn create(client: &mut Client, product: ProductDatabaseInput) -> Result<(), DatabaseError> {
        client
            .execute(
                "INSERT INTO public.products (
                   name,
                   manufacturer,
                   barcode,
                   unit,
                   weight,
                   price,
                   taxable,
                   description,
                   images,
                   sold,
                   added,
                   version
                 ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);",
                &[
                    &product.name,
                    &product.manufacturer,
                    &product.barcode,
                    &product.unit,
                    &product.weight,
                    &product.price,
                    &product.taxable,
                    &product.description,
                    &product.images,
                    &product.sold,
                    &product.added,
                    &product.version,
                ],
            )
            .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }

    pub fn read(client: &mut Client, id: ID) -> Result<Option<Product>, DatabaseError> {
        let row = client
            .query(
                "SELECT products.id, name, manufacturer, barcode,
                        unit, weight, price, taxable, description,
                        images, sold, added, products.version, category
                 FROM products
                 LEFT JOIN product_categories
                 ON products.id = product_categories.product
                 WHERE products.id = $1;",
                &[&id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Product {
                id: row[0].get("id"),
                name: row[0].get("name"),
                manufacturer: row[0].get("manufacturer"),
                barcode: row[0].get("barcode"),
                unit: row[0].get("unit"),
                weight: row[0].get("weight"),
                price: row[0].get("price"),
                taxable: row[0].get("taxable"),
                description: row[0].get("description"),
                images: row[0].get("images"),
                sold: row[0].get("sold"),
                category: row[0].get("category"),
                added: row[0].get("added"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn read_all(client: &mut Client) -> Result<Vec<Product>, DatabaseError> {
        let row = client
            .query(
                "SELECT products.id, name, manufacturer, barcode,
                        unit, weight, price, taxable, description,
                        images, sold, added, products.version, category
                 FROM products
                 LEFT JOIN product_categories
                 ON products.id = product_categories.product;",
                &[],
            )
            .map_err(|_| DatabaseError::SQL)?;
        let mut products: Vec<Product> = vec![];

        for r in &row {
            products.push(Product {
                id: r.get("id"),
                name: r.get("name"),
                manufacturer: r.get("manufacturer"),
                barcode: r.get("barcode"),
                unit: r.get("unit"),
                weight: r.get("weight"),
                price: r.get("price"),
                taxable: r.get("taxable"),
                description: r.get("description"),
                images: r.get("images"),
                category: r.get("category"),
                sold: r.get("sold"),
                added: r.get("added"),
                version: r.get("version"),
            })
        }

        Ok(products)
    }

    pub fn read_by_barcode(
        client: &mut Client,
        barcode: &str,
    ) -> Result<Option<Product>, DatabaseError> {
        let row = client
            .query(
                "SELECT products.id, name, manufacturer, barcode,
                        unit, weight, price, taxable, description,
                        images, sold, added, products.version, category
                 FROM products
                 LEFT JOIN product_categories
                 ON products.id = product_categories.product
                 WHERE barcode = $1;",
                &[&barcode],
            )
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Product {
                id: row[0].get("id"),
                name: row[0].get("name"),
                manufacturer: row[0].get("manufacturer"),
                barcode: row[0].get("barcode"),
                unit: row[0].get("unit"),
                weight: row[0].get("weight"),
                price: row[0].get("price"),
                taxable: row[0].get("taxable"),
                description: row[0].get("description"),
                images: row[0].get("images"),
                sold: row[0].get("sold"),
                category: row[0].get("category"),
                added: row[0].get("added"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn update(
        client: &mut Client,
        id: ID,
        product: &ProductDatabaseInput,
    ) -> Result<(), DatabaseError> {
        let mut transaction = client.transaction().map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE public.products SET name = $1 WHERE id = $2",
                &[&product.name, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET manufacturer = $1 WHERE id = $2",
                &[&product.manufacturer, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET barcode = $1 WHERE id = $2",
                &[&product.barcode, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET unit = $1 WHERE id = $2",
                &[&product.unit, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET weight = $1 WHERE id = $2",
                &[&product.weight, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET price = $1 WHERE id = $2",
                &[&product.price, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET taxable = $1 WHERE id = $2",
                &[&product.taxable, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET description = $1 WHERE id = $2",
                &[&product.description, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET images = $1 WHERE id = $2",
                &[&product.images, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET sold = $1 WHERE id = $2",
                &[&product.sold, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET added = $1 WHERE id = $2",
                &[&product.added, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction
            .execute(
                "UPDATE public.products SET version = $1 WHERE id = $2",
                &[&product.version, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction.commit().map_err(|_| DatabaseError::SQL)?;

        Ok(())
    }

    pub fn delete(client: &mut Client, id: ID) -> Result<(), DatabaseError> {
        client
            .execute(
                "DELETE FROM public.product_categories WHERE product = $1",
                &[&id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        client
            .execute("DELETE FROM public.products WHERE id = $1", &[&id])
            .map_err(|_| DatabaseError::SQL)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Config;
    use crate::database::Database;
    use chrono::Datelike;

    #[test]
    fn test_create_read() {
        let config = Config::from_file("example-config.toml").unwrap();
        let db = Database::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();

        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();

        if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
            let product = ProductDatabaseInput {
                name: "Iphone 15".to_string(),
                manufacturer: "Apple".to_string(),
                barcode: "kdfjdfkdfkdfjdfkj".to_string(),
                unit: Some("grams".to_string()),
                weight: None,
                price: None,
                taxable: Some(false),
                description: "Smart phone".to_string(),
                images: "".to_string(),
                sold: false,
                added: date,
                version: 0,
            };

            if let Some(mut client) = db.client {
                super::ProductsSql::create(&mut client, product.clone()).unwrap();
                let product1 =
                    super::ProductsSql::read_by_barcode(&mut client, &product.barcode).unwrap();

                if let Some(product1) = product1 {
                    assert_eq!(product.name, product1.name);

                    if let Some(product2) =
                        super::ProductsSql::read(&mut client, product1.id).unwrap()
                    {
                        assert_eq!(product.name, product2.name);
                        assert_eq!(product1, product2);
                    }
                } else {
                    panic!("Product not found");
                }
            } else {
                panic!("woop");
            }
        } else {
            panic!("Date error");
        }
    }

    #[test]
    fn test_update_delete() {
        let config = Config::from_file("example-config.toml").unwrap();
        let db = Database::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();

        let current_date = chrono::Utc::now();
        let year = current_date.year();
        let month = current_date.month();
        let day = current_date.day();

        if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
            let product = ProductDatabaseInput {
                name: "Iphone 12".to_string(),
                manufacturer: "Apple".to_string(),
                barcode: "wdjfkasfhd7afd,df".to_string(),
                unit: Some("grams".to_string()),
                weight: None,
                price: None,
                taxable: Some(false),
                description: "Smart phone".to_string(),
                images: "".to_string(),
                sold: false,
                added: date,
                version: 0,
            };

            let product_update = ProductDatabaseInput {
                name: "Iphone 11".to_string(),
                manufacturer: "Apple".to_string(),
                barcode: "wdjfkasfhd7afd,df".to_string(),
                unit: Some("grams".to_string()),
                weight: None,
                price: None,
                taxable: Some(false),
                description: "Smart phone".to_string(),
                images: "".to_string(),
                sold: false,
                added: date,
                version: 0,
            };

            if let Some(mut client) = db.client {
                super::ProductsSql::create(&mut client, product.clone()).unwrap();
                super::ProductsSql::update(&mut client, 1, &product_update.clone()).unwrap();
                if let Some(updated) =
                    super::ProductsSql::read_by_barcode(&mut client, &product_update.barcode)
                        .unwrap()
                {
                    assert_eq!(product_update.name, updated.name);

                    // delete product
                    super::ProductsSql::delete(&mut client, updated.id).unwrap();
                    let deleted_product =
                        super::ProductsSql::read(&mut client, updated.id).unwrap();
                    assert_eq!(deleted_product, None);
                } else {
                    panic!("tring");
                }
            } else {
                panic!("woop");
            }
        } else {
            panic!("Error");
        }
    }
}
