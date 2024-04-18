

arenaW = 320
arenaH = 240
ballW = 20
ballH = 20

--
--class Ball:
--    def __init__(self, x: int, y: int):
--        self._x = x
--        self._y = y
--        self._dx = 5
--        self._dy = 5
--    #...
--

data Ball = Ball { x :: Int
                     , y :: Int
                     , dx :: Int
                     , dy :: Int
                     } deriving (Eq, Show, Read)

--work in progress