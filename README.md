# Razor

## Descrição
Razor é a fundação modular e escalável para o desenvolvimento de jogos em Rust, projetada para ser o núcleo de frameworks e engines mais complexas. Este projeto oferece os sistemas essenciais para renderização 2D e 3D, gerenciamento de física, ECS (Entity Component System), input, e áudio e demais sistemas. 

O Razor serve como base e partida para a construção de um framework e uma engine customizada, oferecendo alta performance e flexibilidade. Também poderá ser utilizado para construção de outros projetos similares ou com outras propostas. Utiliza a licença MIT, além de ser leve e modular, permitindo que desenvolvedores expandam ou ajustem o código para suas necessidades específicas.

## Principais Funcionalidades
- **Renderização 2D/3D**: Sistema gráfico utilizando wgpu e geez, permitindo renderizações eficientes e multi-plataforma.
- **Física 2D/3D**: Suporte a simulações físicas, utilizando bibliotecas como rapier para oferecer colisão e movimentações realistas.
- **ECS (Entity Component System)**: Sistema para gerenciar entidades e seus componentes, facilitando o desenvolvimento de grandes sistemas.
- **Input**: Gerenciamento de eventos de teclado, mouse e gamepads, utilizando a biblioteca winit.
- **Áudio**: Sistema para o gerenciamento de som e música, utilizando rodio e outras ferramentas.
- **Modularidade extrema**: Projete seus próprios sistemas ou adicione novos módulos de acordo com a necessidade.

## Objetivo do Projeto
O Razor é a fundação técnica para a criação de frameworks e engines de jogos. Ele será utilizado como a base para dois futuros projetos próprios:
1. **Razorlib**: Um framework de desenvolvimento rápido de jogos em Rust, mais completo, otimizado e robusto, similar ao libGDX e a outros frameworks.
2. **Razor Engine**: Uma engine de jogos completa com editor visual, facilitando o desenvolvimento de jogos para diversas plataformas.

## Estrutura do Projeto
- `/src`: Código fonte
- `/docs`: Documentação
- `/tests`: Testes automatizados
- `/config`: Arquivos de configuração

Como Usar:

## Requisitos
- Rust (versão 1.60 ou superior)
- Cargo
- Bibliotecas previstas (Wgpu,Ggez,Winit,rapier,rodio) sera 
especificádo comoforme o desenvolvimento  
    
Instalação:

Clone este repositório, instale as dependências com o Cargo e avalie se as
bibliotecas estão atualizadas e ajustadas.

# bash
```
git clone [url do repositorio]
cd razor-base
cargo build
cargo run
```


Exemplos:

Aqui estão exemplos de como utilizar os principais sistemas (renderização, física, ECS, etc.). [Em construção]

Contribuições:

Contribuições são bem-vindas! Se você quiser ajudar no desenvolvimento ou sugerir melhorias, fique à vontade para abrir uma issue ou enviar um pull request. [Em construção]

Licença
Este projeto está licenciado sob a licença **MIT**. Veja o arquivo LICENSE para mais detalhes.
