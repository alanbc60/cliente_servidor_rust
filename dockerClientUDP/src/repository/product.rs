use std::process::id;

use crate::domain;
use crate::repository;
use postgres::SimpleQueryMessage;

impl repository::Repository{
    /*pub fn all_products(&mut self) -> Vec<domain::Product> {
        const SELECT_ALL: &str = "SELECT myid, nombreReporte FROM reportes ORDER BY nombreReporte";
        let mut products: Vec<domain::Product> = Vec::new();
        let result = self.db.simple_query(SELECT_ALL).unwrap();

        for data in result  {
            if let SimpleQueryMessage::Row(row) = data {
                let product = domain::Product{
                    id: row.get(0).unwrap().parse().unwrap(),
                    nombreReporte: row.get(1).unwrap().to_string(),

                };
                products.push(product);
            }
        };
        products
    }*/

    pub fn consulta_por_nombre(&mut self, value: &str) -> Vec<domain::Product>{
        const SELECT_nombreReporte_START:&str = 
            "SELECT myid, nombreReporte FROM reportes WHERE nombreReporte ilike $1::varchar ORDER BY nombreReporte";
        let mut products: Vec<domain::Product> = Vec::new();
        let value = format!("{}%",value);
        let result = self.db.query(SELECT_nombreReporte_START, &[&value]).unwrap();
        for row in result{
            let product = domain::Product{
                id: row.get(0),
                nombreReporte: row.get(1),
            };
            products.push(product);
        }
        products
    }

    pub fn consulta_informe(&mut self, value: &str) -> Vec<domain::Product2>{
        const SELECT_nombreReporte_START:&str = 
            "SELECT myid, informeProblema FROM reportes WHERE informeProblema ilike $1::varchar ORDER BY informeProblema";
        let mut products: Vec<domain::Product2> = Vec::new();
        let value = format!("{}%",value);
        let result = self.db.query(SELECT_nombreReporte_START, &[&value]).unwrap();
        
        for row in result{
            let product = domain::Product2{
                id: row.get(0),
                informeProblema: row.get(1),
            };
            products.push(product);
        }
        products
    }

    pub fn consulta_id(&mut self, value: &str) -> Vec<domain::Product>{
        const SELECT_nombreReporte_START :  &str =  
            "SELECT myid, nombreReporte FROM reportes WHERE myid = $1::integer";
            let mut products: Vec<domain::Product> = Vec::new();
        let myid: i32 = value.parse().unwrap();
        let result = self.db.query(SELECT_nombreReporte_START, &[&myid]).unwrap();
        for row in result{
            let product = domain::Product{
                id: row.get(0),
                nombreReporte: row.get(1),
            };
            products.push(product);
        }
        products
    }

}