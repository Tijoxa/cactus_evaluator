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
[Cactus Kev Algorithm](http://suffe.cool/poker/7462.html)