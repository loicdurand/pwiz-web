document.addEventListener('DOMContentLoaded', () => {

  // Tooltips init
  const tooltipped = document.querySelectorAll('.tooltipped');
  M.Tooltip.init(tooltipped);

  // Floating action button init
  const actions_buttons = document.querySelectorAll('.fixed-action-btn');
  M.FloatingActionButton.init(actions_buttons);

  // Modal init
  const modals = document.querySelectorAll('.modal');
  M.Modal.init(modals);

  // Chips init
  const chips = document.querySelectorAll('.chips');
  const [chips_instance] = M.Chips.init(chips);
  const tags_input = document.querySelector('.tags-input');
  const hidden = document.getElementById('tags');
  if (hidden !== null) {
    const values = tags_input.dataset.chips.split(' ');
    values.forEach(value => chips_instance.addChip({ tag: value }));
    tags_input.addEventListener('keyup', () => {
      hidden.value = chips_instance.chipsData.map(chip => chip.tag).join(' ');
    });

    chips.forEach(chip => {
      chip.addEventListener('click', ({ target }) => {
        if (target.matches('.close')) {
          hidden.value = chips_instance.chipsData.map(chip => chip.tag).join(' ');
        }
      });
    });
  }

  // Recherche
  const search = document.getElementById('tutoriel-search');
  search?.addEventListener('keyup', filter);
  search?.addEventListener('keydown', filter);

  // Click sur le contenu d'un tutoriel
  const links = document.querySelectorAll('.link');
  links?.forEach(link => {
    const href = link.dataset.href;
    link.addEventListener('click', () => {
      window.location.href = href;
    });
  });

  // Click sur le bouton de suppression d'un tutoriel
  const deleteButtons = document.querySelectorAll('.confirm-delete');
  deleteButtons?.forEach(button => {
    const href = button.getAttribute('href');
    const id = href.split('/').pop();
    button.addEventListener('click', (e) => {
      e.preventDefault();
      xhr_delete(href)
        .then(() => {
          const card = document.getElementById(`card-${id}`);
          card.remove();
          M.toast({ html: 'Tutoriel supprimé avec succès', displayLength: 1e4 });
          if (location.pathname.startsWith('/afficher'))
            setTimeout(() => {
              location.href = "/";
            }, 1200);
        });

    });
  });

  // const tags = document.querySelector('.tags-input');
  // console.log({tags});
  // tags.addEventListener('input', () => {
  //   const chips = M.Chips.getInstance(tags);

  //   // const value = chips.chipsData.map(chip => chip.tag).join(' ');
  //   console.log({chips});
  // });

});

function filter({ target }) {
  const value = target.value.toLowerCase().trim();
  const cards = document.querySelectorAll('.tutoriel');
  cards.forEach(card => {
    const tags = card.dataset.tags;
    const re = new RegExp(value, "i");
    if (value === "" || re.test(tags)) {
      card.style.display = 'block';
    } else {
      card.style.display = 'none';
    }
  });
}

async function xhr_delete(url) {
  try {
    const // 
      options = {
        method: 'delete',
        headers: {}
      },
      response = await fetch(url, options);

    if (!response.ok) {
      const message = 'Error with Status Code: ' + response.status;
      throw new Error(message);
    }

    return {};

  } catch (error) {
    console.log(error);
  }
}