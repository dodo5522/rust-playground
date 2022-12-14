def main(result: str) -> None:
  print(result)

  rs = result.split(',')
  results = {artist: rs.count(artist) for artist in set(rs)}
  for artist, counter in results.items():
    print(f'{artist}: {counter}')


if __name__ == '__main__':
  main('C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C')
