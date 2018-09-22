#![feature(specialization)]

extern crate numpy;
extern crate pyo3;
extern crate rand;

mod game;

use numpy::{PyArray, PyArrayModule};
use pyo3::prelude::*;

#[pyclass]
struct Game {
    game: game::Game,
}

#[pymethods]
impl Game {
    #[new]
    fn __new__(obj: &PyRawObject) -> PyResult<()> {
        obj.init(|_| Game {
            game: game::Game::new(),
        })
    }

    fn step(&mut self, direction: u8) -> PyResult<(usize, bool)> {
        let direction = match direction {
            0 => game::Direction::Left,
            1 => game::Direction::Up,
            2 => game::Direction::Right,
            3 => game::Direction::Down,
            _ => return Err(exc::ValueError::new("Invalid direction")),
        };
        let (next, reward, done) = self.game.step(&direction);
        self.game = next;
        Ok((reward, done))
    }

    //#[getter]
    fn board(&self, py: Python) -> PyResult<PyArray<u8>> {
        let np = PyArrayModule::import(py)?;
        Ok(PyArray::from_vec(py, &np, self.game.board.to_vec()))
    }
}

#[pyproto]
impl PyObjectProtocol for Game {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.game))
    }
}

#[pymodinit]
fn zwovieracht(py: Python, m: &PyModule) -> PyResult<()> {
    let _np = PyArrayModule::import(py)?;

    m.add_class::<Game>()?;

    Ok(())
}
