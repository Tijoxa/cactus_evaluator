# cactus_evaluator
Implémentation de l'algorithme Cactus Kev Perfect Hash.

# Dépendances
- [Rust](https://www.rust-lang.org/fr)
- maturin (pip install maturin)

# Utilisation
- maturin develop
- Utilisation dans Python :
```Python
import cactus_evaluator
cactus_evaluator.evaluate('AC', '4C', '5C', 'TC', 'AH')
cactus_evaluator.evaluate_7('AC', '4C', '5C', 'TC', 'AH', '9H', '7S')
```

# Source
- [Cactus Kev Algorithm](https://suffe.cool/poker/evaluator.html)
- [Hash Tables](http://suffe.cool/poker/7462.html)
- [PyO3](https://github.com/PyO3/pyo3)